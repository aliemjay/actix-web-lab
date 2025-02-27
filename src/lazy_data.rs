use std::{
    cell::Cell,
    fmt,
    future::{ready, Future, Ready},
    rc::Rc,
};

use actix_web::{dev, error, Error, FromRequest, HttpRequest};
use futures_core::future::LocalBoxFuture;
use tokio::sync::OnceCell;

/// A lazy extractor for thread-local data.
pub struct LazyData<T> {
    inner: Rc<LazyDataInner<T>>,
}

pub struct LazyDataInner<T> {
    cell: OnceCell<T>,
    fut: Cell<Option<LocalBoxFuture<'static, T>>>,
}

impl<T> Clone for LazyData<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for LazyData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lazy")
            .field("cell", &self.inner.cell)
            .field("fut", &"..")
            .finish()
    }
}

impl<T> LazyData<T> {
    pub fn new<F, Fut>(init: F) -> LazyData<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T> + 'static,
    {
        Self {
            inner: Rc::new(LazyDataInner {
                cell: OnceCell::new(),
                fut: Cell::new(Some(Box::pin(init()))),
            }),
        }
    }

    pub async fn get(&self) -> &T {
        self.inner
            .cell
            .get_or_init(|| async move {
                match self.inner.fut.take() {
                    Some(fut) => fut.await,
                    None => panic!("LazyData instance has previously been poisoned"),
                }
            })
            .await
    }
}

impl<T: 'static> FromRequest for LazyData<T> {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        if let Some(lazy) = req.app_data::<LazyData<T>>() {
            ready(Ok(lazy.clone()))
        } else {
            log::debug!(
                "Failed to extract `LazyData<{}>` for `{}` handler. For the Data extractor to work \
                correctly, wrap the data with `LazyData::new()` and pass it to `App::app_data()`. \
                Ensure that types align in both the set and retrieve calls.",
                core::any::type_name::<T>(),
                req.match_name().unwrap_or_else(|| req.path())
            );

            ready(Err(error::ErrorInternalServerError(
                "Requested application data is not configured correctly. \
                View/enable debug logs for more details.",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use actix_web::{
        http::StatusCode,
        test::{call_service, init_service, TestRequest},
        web, App, HttpResponse,
    };

    use super::*;

    #[actix_web::test]
    async fn lazy_data() {
        let app = init_service(
            App::new()
                .app_data(LazyData::new(|| async { 10usize }))
                .service(web::resource("/").to(|_: LazyData<usize>| HttpResponse::Ok())),
        )
        .await;
        let req = TestRequest::default().to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let app = init_service(
            App::new()
                .app_data(LazyData::new(|| async {
                    actix_web::rt::time::sleep(Duration::from_millis(40)).await;
                    10usize
                }))
                .service(web::resource("/").to(|_: LazyData<usize>| HttpResponse::Ok())),
        )
        .await;
        let req = TestRequest::default().to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let app = init_service(
            App::new()
                .app_data(LazyData::new(|| async { 10u32 }))
                .service(web::resource("/").to(|_: LazyData<usize>| HttpResponse::Ok())),
        )
        .await;
        let req = TestRequest::default().to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn lazy_data_web_block() {
        let app = init_service(
            App::new()
                .app_data(LazyData::new(|| async {
                    web::block(|| std::thread::sleep(Duration::from_millis(40)))
                        .await
                        .unwrap();

                    10usize
                }))
                .service(web::resource("/").to(|_: LazyData<usize>| HttpResponse::Ok())),
        )
        .await;
        let req = TestRequest::default().to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
