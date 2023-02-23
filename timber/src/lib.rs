use tracing::info;

#[macro_export]
macro_rules! timber {
    ($($arg:tt)*) => {{
      info!("{}", std::format_args!($($arg)*));
    }};
  }

#[macro_export]
macro_rules! err_without {
  ( $e:expr ) => {
    match $e {
      Ok(x) => x,
      Err(e) => {
        timber!("Fatal Error: {:?}", e);
        return Err(());
      }
    }
  };
}

pub enum ErrorCode {
  ExpectationFailed,
  InternalServerError,
  NotFound,
  ServiceUnavailable,
}
