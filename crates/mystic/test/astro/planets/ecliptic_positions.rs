use mystic::astro::planets::{
	ecliptic_positions::{ecliptic_position, jupiter},
	*,
};
use sweet::*;



//calculator: http://cosinekitty.com/solar_system.html
sweet! {

	before{
	}

	test "ecliptic positions"{
		let day = DAY_1_JAN_2000;
		expect(ecliptic_positions::sun())
			.to_be(RectangluarCoords { x: 0., y: 0., z: 0. })?;
		expect(ecliptic_positions::moon(day))
			.to_be(RectangluarCoords { x: -0.17071703140942265, y: 0.9671373403660894, z: 0.000244455888348224 })?;
		expect(ecliptic_positions::mercury(day))
			.to_be(RectangluarCoords { x: -0.14072937612299719, y: -0.44390213113733545, z: -0.023343754406466743 })?;
		expect(ecliptic_positions::venus(day))
			.to_be(RectangluarCoords { x: -0.7186427978467534, y: -0.022558508614733574, z: 0.041172246944286506 })?;
		expect(ecliptic_positions::earth(day))
			.to_be(RectangluarCoords { x: -0.16859614540661194, y: 0.9687542029346362, z: 0.0 })?;
		expect(ecliptic_positions::jupiter(day))
			.to_be(RectangluarCoords { x: 4.004410412346112, y: 2.9373123101736742, z: -0.10175564366099876 })?;
		expect(ecliptic_positions::saturn(day))
			.to_be(RectangluarCoords { x: 6.407513191982934, y: 6.565128118257096, z: -0.3699571287452595 })?;
		expect(ecliptic_positions::uranus(day))
			.to_be(RectangluarCoords { x: 14.425425328151979, y: -13.737612206413395, z: -0.23802068662353199 })?;
		expect(ecliptic_positions::neptune(day))
			.to_be(RectangluarCoords { x: 16.796170232469972, y: -24.982713961883295, z: 0.12733440092952736})?;
		expect(ecliptic_positions::pluto(day))
			.to_be(RectangluarCoords { x: -9.892380757344934, y: -27.99251636591456, z: 5.858940692166365 })?;
	}

	test skip "print jpl"{

		let jup: ecliptic_positions::OrbitalConstants = ecliptic_positions::JUPITER_JPL.into();
		let sat: ecliptic_positions::OrbitalConstants = ecliptic_positions::SATURN_JPL.into();
		let urn: ecliptic_positions::OrbitalConstants = ecliptic_positions::URANUS_JPL.into();

		println!("jupiter {:?}", jup);
		println!("saturn {:?}", sat);
		println!("uranus {:?}", urn);

	}
}


/*
http://cosinekitty.com/solar_system.html

Sun 0.0000000	0.0000000	0.0000000
Mercury -0.1407294	-0.4439021	-0.0233438
Venus -0.7186428	-0.0225585	0.0411722
Earth -0.1685961	0.9687542	0.0000000
Moon -0.1707170	0.9671373	0.0002445
Mars 1.3902750	-0.0209665	-0.0346094	1.3908637	1.04	47.7°
Jupiter 4.0044104	2.9373123	-0.1017556
Saturn 6.4075132 6.5651281	-0.3699571
Uranus 14.4254253	-13.7376122	-0.2380207
Neptune 16.7961702 -24.9827140	0.1273344	30.1042114
Pluto -9.8923808 -27.9925164	5.8589407

*/
