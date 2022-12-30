#[cfg(test)]
mod tests {
    use webserver::Router;

    #[test]
    fn test_fn_call(){
        let function_name = "test_fn";
        Router::call(function_name)
    }

}
