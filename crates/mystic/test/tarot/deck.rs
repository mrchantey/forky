use mystic::tarot::spread::*;
use mystic::*;
use sweet::*;

sweet! {
	it "works" {

		let mut deck = tarot::TarotDeck::default();

		expect(deck.len()).to_be(tarot::NUM_TAROT_CARDS.into())?;

		for (i,card) in deck.iter().enumerate(){
			expect(i).to_be(card.index() as usize)?;
		}

		deck.shuffle();

		// let spread = tarot::spread::PastPresentFuture::new();


		// println!("{}", spread.print());

		// expect(true).to_be_false()?;

	}
}
