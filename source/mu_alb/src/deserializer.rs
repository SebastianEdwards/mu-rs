use aws_lambda_events::event::alb::{
    AlbTargetGroupRequest as Request
};

use mu_runtime::Context;
use mu_runtime::Error;

pub trait AlbDeserialize<T> {
    fn from_alb_request(req: Request, ctx: Context) -> Result<(T, Context), Error>;
}

impl AlbDeserialize<Request> for Request {
    fn from_alb_request(
        req: Request,
        ctx: Context,
    ) -> Result<(Request, Context), Error> {
        Ok((req, ctx))
    }
}

pub trait RpcRequest {}

impl<T> AlbDeserialize<T> for T
where
    T: for<'de> serde::Deserialize<'de> + RpcRequest,
{
    fn from_alb_request(req: Request, _ctx: Context) -> Result<(T, Context), Error> {
        match &req.body {
            Some(body) => match serde_json::from_str(body) {
                Ok(deserialized) => Ok(deserialized),
                Err(cause) => Err(format!("Failed {:?}", cause).into()),
            },
            None => Err("No payload defined".into()),
        }
    }
}
