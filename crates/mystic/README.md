# Mystic

This crate can be used either as a CLI, see `examples/cli`, or as a library. It uses ChatGPT for interpretation, but this is not required. If you are using that then you'll need a `.env` containing your `CHATGPT_KEY`, ie:

```sh
#.env at root
CHATGPT_KEY=foo
```

## Supported Categories

### Tarot
```rs
	let mut deck = TarotDeck::new();
	deck.shuffle();
	let spread = spread::PastPresentFuture::new(&mut deck);

	let gpt = ChatGptInstance::new()?;
	println!("interpreting your spread...\n{}", spread.print());
	let result = spread.interpret(&gpt).await?;
```