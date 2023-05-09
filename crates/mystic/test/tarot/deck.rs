use mystic::tarot::constants::TarotCardEnum;
use mystic::tarot::spread::*;
use mystic::*;
use strum::EnumCount;
use sweet::*;

sweet! {
	it "works" {

		let mut deck = tarot::TarotDeck::default();

		expect(deck.len()).to_be(TarotCardEnum::COUNT)?;

		for (i,card) in deck.iter().enumerate(){
			expect(i).to_be(card.into())?;
		}

		// deck.shuffle();

		// let spread = tarot::spread::PastPresentFuture::new();


		// println!("{}", spread.print());

		// expect(true).to_be_false()?;

	}
}
