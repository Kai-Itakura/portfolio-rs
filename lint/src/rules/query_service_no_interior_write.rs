use syn::ExprMethodCall;

use super::Rule;
use crate::context::Context;

const WRITE_METHODS: &[&str] = &["write", "lock", "borrow_mut"];

pub struct QueryServiceNoInteriorWrite;

impl Rule for QueryServiceNoInteriorWrite {
    fn check_expr_method_call(&self, cx: &mut Context<'_>, expr: &ExprMethodCall) {
        if !cx.in_query_service {
            return;
        }
        let method = expr.method.to_string();
        if WRITE_METHODS.contains(&method.as_str()) {
            cx.report(
                expr.method.span(),
                "query-service-no-interior-write",
                format!(
                    "クエリサービスで `.{method}()` を呼んでいる。クエリサービスは状態を変えないので、`RwLock` は `.read()`、`RefCell` は `.borrow()` で読む"
                ),
            );
        }
    }
}
