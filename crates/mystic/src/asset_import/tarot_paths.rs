use crate::tarot::{constants::TarotCardEnum, TarotCard};
use std::collections::HashMap;
use strum::EnumCount;




pub fn get_tarot_cards() -> HashMap<TarotCard, &'static str> {
	let mut map = HashMap::with_capacity(TarotCardEnum::COUNT);
	for i in 0..TarotCardEnum::COUNT {
		map.insert(i.try_into().unwrap(), TAROT_PATHS[i]);
	}

	map
}

//TODO minor suits may be out of order
pub const TAROT_PATHS:[&'static str;TarotCardEnum::COUNT] = [
	"http://blogimg.goo.ne.jp/user_image/07/b1/6ebb2d3d2427526a2e5b1318a56c1b70.jpg",
	"http://blogimg.goo.ne.jp/user_image/77/cb/481628b008f6379a765139553c4db783.jpg",
	"http://blogimg.goo.ne.jp/user_image/46/fc/2cf480c9bfd704f816b73911a620f44c.jpg",
	"http://blogimg.goo.ne.jp/user_image/05/7f/ac3e5a25130cf5ce712502e48c094ef1.jpg",
	"http://blogimg.goo.ne.jp/user_image/2c/17/81571019272e8c25f754e42702e52765.jpg",
	"http://blogimg.goo.ne.jp/user_image/4d/fc/0396dfed9a85453ec33d5e70bcb07737.jpg",
	"http://blogimg.goo.ne.jp/user_image/1b/48/f03aa13e7405ee7a90c07e85917c3d68.jpg",
	"http://blogimg.goo.ne.jp/user_image/0c/56/d99ac829d630a9583491cb72de8d85e9.jpg",
	"http://blogimg.goo.ne.jp/user_image/58/07/ad0d3139093e276d30d2ecbf032f5230.jpg",
	"http://blogimg.goo.ne.jp/user_image/6f/12/647c259f0e8f970844cca76802007a06.jpg",
	"http://blogimg.goo.ne.jp/user_image/0b/94/b555ff4834aa8302ca12ce37f0f2a942.jpg",
	"http://blogimg.goo.ne.jp/user_image/73/8d/dde7d3947d8cc0b8e10f97cdd2ebdedd.jpg",
	"http://blogimg.goo.ne.jp/user_image/5c/f5/373990dc7703fd8e2007064a4e476788.jpg",
	"http://blogimg.goo.ne.jp/user_image/04/e3/f059d2452469555b041c6d88f211d054.jpg",
	"http://blogimg.goo.ne.jp/user_image/52/f4/faf93626b613cdf6fd45f183ac493007.jpg",
	"http://blogimg.goo.ne.jp/user_image/6c/bf/69cc9a4f46a918f3f65983b32b1ebf20.jpg",
	"http://blogimg.goo.ne.jp/user_image/1d/8f/0f21a5cd7b2b3034e58ebb7a0cd98008.jpg",
	"http://blogimg.goo.ne.jp/user_image/39/92/37e9377240ed5416b7b124fb123f54cb.jpg",
	"http://blogimg.goo.ne.jp/user_image/6a/2e/8e27e2dd144957196469a3c423fe1fb4.jpg",
	"http://blogimg.goo.ne.jp/user_image/16/a3/7012cbdb9b5466885931e39832ae6c5e.jpg",
	"http://blogimg.goo.ne.jp/user_image/70/6f/6d4976f94d29d6ba0d30d4992eb50ba1.jpg",
	"http://blogimg.goo.ne.jp/user_image/39/e1/cd146c178e0f7eb196a3d6a43d719d09.jpg",
	"http://blogimg.goo.ne.jp/user_image/40/81/8d95a43b697812434166ee8f27a720ac.jpg",
	"http://blogimg.goo.ne.jp/user_image/3b/2d/004365cc22e163bdcc2d21fdfde23d7d.jpg",
	"http://blogimg.goo.ne.jp/user_image/04/50/845f8a9fb7bfdd864a8e5dd37a328f63.jpg",
	"http://blogimg.goo.ne.jp/user_image/6f/ec/397cb90176802df7231f1adc25101067.jpg",
	"http://blogimg.goo.ne.jp/user_image/38/99/ba99478f1724269574ed228d1fe65d39.jpg",
	"http://blogimg.goo.ne.jp/user_image/65/d0/f43f052989ea467c8cd98ec6ccea6236.jpg",
	"http://blogimg.goo.ne.jp/user_image/6b/04/f3d5253a2ecb3e7567aa916233df9df1.jpg",
	"http://blogimg.goo.ne.jp/user_image/17/c6/558b4e8cdf370aa804b767d3b354a6f3.jpg",
	"http://blogimg.goo.ne.jp/user_image/03/08/8c76cf2f48d272cc19b47f97ba81796b.jpg",
	"http://blogimg.goo.ne.jp/user_image/7b/c2/527e31fc4e70508714c886bf96ff63d1.jpg",
	"http://blogimg.goo.ne.jp/user_image/0e/81/609a9dc57fd9985291eb66a3cf8dca6a.jpg",
	"http://blogimg.goo.ne.jp/user_image/7a/49/6c46cb9983f3515c4cb1cefc060876e6.jpg",
	"http://blogimg.goo.ne.jp/user_image/09/04/7d1f6ee8135648f05483fa4464d2c8ca.jpg",
	"http://blogimg.goo.ne.jp/user_image/00/5b/0cec8f7fa157a38b0f0cedc07b32e4ab.jpg",
	"http://blogimg.goo.ne.jp/user_image/76/6b/06f3f41b220c4d916e10715796677bde.jpg",
	"http://blogimg.goo.ne.jp/user_image/3b/d5/99b3b0f6c866551bdc27a29aaecb4247.jpg",
	"http://blogimg.goo.ne.jp/user_image/2b/fb/ebc3f5e87e5b709a48db9da9e79e1e43.jpg",
	"http://blogimg.goo.ne.jp/user_image/06/b0/5508140ebd01961145be264b3268ff2a.jpg",
	"http://blogimg.goo.ne.jp/user_image/6a/98/2927fee2fde0aefd0c330314f89bf7bb.jpg",
	"http://blogimg.goo.ne.jp/user_image/14/a3/241d15a8ad3267c539d04fd757d9e542.jpg",
	"http://blogimg.goo.ne.jp/user_image/5b/8d/bb8d98bf0f7203a3e4c8298e4a9a1b22.jpg",
	"http://blogimg.goo.ne.jp/user_image/7f/dc/d2605ce6808525009f9a28940cb058c8.jpg",
	"http://blogimg.goo.ne.jp/user_image/03/fd/b6a47a05b7e1f65c5a11fdb94925124a.jpg",
	"http://blogimg.goo.ne.jp/user_image/1f/ee/c81848059b9a89463c46bdcd2c88c599.jpg",
	"http://blogimg.goo.ne.jp/user_image/79/0a/801f7bf8eee3301e6202a560cc53c9f1.jpg",
	"http://blogimg.goo.ne.jp/user_image/77/91/2bebe4622ad85c4d90dbcdf4ce31fe1c.jpg",
	"http://blogimg.goo.ne.jp/user_image/27/be/fa90a47ee73ba721f168a88dbde6a0ae.jpg",
	"http://blogimg.goo.ne.jp/user_image/1d/9e/c8aa12b866e297efc9434c03cdd7396f.jpg",
	"http://blogimg.goo.ne.jp/user_image/3c/23/f8cb84fb8664f6abdbd20de24d462b15.jpg",
	"http://blogimg.goo.ne.jp/user_image/0e/25/bb491398391f6eaaa5af34ad4d994348.jpg",
	"http://blogimg.goo.ne.jp/user_image/12/cb/4b675afade50605ba5a577fbe8e577d0.jpg",
	"http://blogimg.goo.ne.jp/user_image/34/73/b1d410948bfd3f6a78a10b300ab4ddba.jpg",
	"http://blogimg.goo.ne.jp/user_image/46/3e/0be3262628943245e88c0a738e912670.jpg",
	"http://blogimg.goo.ne.jp/user_image/10/fd/31107595222497ec0eccc1922385740b.jpg",
	"http://blogimg.goo.ne.jp/user_image/7d/ce/c98c3a34e49005cb22e436d498bdb937.jpg",
	"http://blogimg.goo.ne.jp/user_image/19/10/782d330316b1e4f01ffe9bcea723582d.jpg",
	"http://blogimg.goo.ne.jp/user_image/66/7d/c40e9587405e8027cbd49ea69073fff2.jpg",
	"http://blogimg.goo.ne.jp/user_image/7b/55/63e4b7d92a645b294eebd69a83c087ce.jpg",
	"http://blogimg.goo.ne.jp/user_image/78/79/fac3b4028c341f99fefd0fee1b849b28.jpg",
	"http://blogimg.goo.ne.jp/user_image/74/63/8efdaf16d33c4dd6ef737931e28c0ba8.jpg",
	"http://blogimg.goo.ne.jp/user_image/6c/60/0992d7ef9961e43f8e25d5f7911b7ae5.jpg",
	"http://blogimg.goo.ne.jp/user_image/4c/49/15b8c919c4c7f09822639a56b0f2ae11.jpg",
	"http://blogimg.goo.ne.jp/user_image/51/bc/41b0352ed748ec8ce5b886a98c2702f1.jpg",
	"http://blogimg.goo.ne.jp/user_image/0e/45/76b757c4e1823ed6a159dc0f0d792591.jpg",
	"http://blogimg.goo.ne.jp/user_image/1d/a6/1f848ad29c33fd0dbc4d9e1b9062cfa2.jpg",
	"http://blogimg.goo.ne.jp/user_image/19/ed/ab47e24e9edf8b9a28644603c483536b.jpg",
	"http://blogimg.goo.ne.jp/user_image/53/3c/db8d42570a820cb5a6dbf5a3f8de7c8d.jpg",
	"http://blogimg.goo.ne.jp/user_image/2b/45/8e67d4db3c6d4a112d60a4502b620033.jpg",
	"http://blogimg.goo.ne.jp/user_image/49/da/d1b1b5df99b0346ddc70c933dc14d761.jpg",
	"http://blogimg.goo.ne.jp/user_image/06/82/b1cfc33a0ecdda14045f29ead54200ac.jpg",
	"http://blogimg.goo.ne.jp/user_image/61/34/752c046606f05c09e5df5ebfa1090315.jpg",
	"http://blogimg.goo.ne.jp/user_image/51/e3/e23b7d980f6bf1951cfbf186d8108ead.jpg",
	"http://blogimg.goo.ne.jp/user_image/40/9e/d2c7bb55c36222a2e140aff440d4c48f.jpg",
	"http://blogimg.goo.ne.jp/user_image/2e/a2/12522ae5f5015570e3054cce690b8804.jpg",
	"http://blogimg.goo.ne.jp/user_image/28/d8/e455de31968f0b18d916e2123abdb355.jpg",
	"http://blogimg.goo.ne.jp/user_image/5d/70/a1ad3ce94b7a1064032c9e221947c046.jpg",
];
