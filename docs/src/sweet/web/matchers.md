# Matchers


## `expect(element)`
Querying a html element is so common Sweet has matchers for some common checks:

- `to_contain_html`
- `to_contain_text`
- `to_contain_visible_test`

## `IntoHtmlElement`

`window().unwrap().document().unwrap().body().unwrap()` is a bit of a mouthful 🥴

Sweet provides some wrappers around common types, ie `web_sys::window`:

```rs
//window implements IntoHtmlElement
expect(web_sys::window).to_contain_text("sweet as!")?;

//so does iframe
let page = visit("localhost:7777").await; 
expect(page).to_contain_text("sweet as!")?;
```


## Async Matchers
Lots of web stuff happens at weird times, so we've got helpers like `poll()`, which will wait for 2 seconds before failing.

```rs
expect(page).poll(|p|
	p.to_contain_text("sweet as!")).await?;
```

We can also retrieve child elements via polling
```rs
expect(page).poll(|p| p.get("p")).await?
	.to_contain_text("sweet as!")?;
```
