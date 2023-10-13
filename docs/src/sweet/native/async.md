# Async Tests

Sweet allows async tests but cannot tell whether all awaited futures are `Send`. 

This is solved by adding the `non_send` attribute:
```rs

// many async functions are parallelizable
#[sweet_test]
async fn example_parrallelizable_test(){
	tokio::time::sleep(Duration::from_millis(100)).await.unwrap();
}


// some must be run on the main thread
#[sweet_test(non_send)]
async fn example_non_send_tests(){
	fantoccini::ClientBuilder::native().connect("http://example.com").await;
}
```