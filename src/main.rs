fn main() {
	let sentence = String::from("this is normal on english");
	let sentence = sentence.trim();
	let sentence = sentence.to_string() + " ";
	let mut pig_latin = String::new();
	let mut last:char = ' ';
	let mut last_index = 0;
	let mut to_pop = false;

	let mut added:usize = 0;

  	for (index, a) in sentence.chars().enumerate() {
		if a == ' '{
			if (last == 'a' || last == 'e' ||last == 'i' ||last == 'o' ||last == 'u') {
				let index = last_index  + added;
				if pig_latin.is_char_boundary(index) {
					pig_latin.insert(index, last);
					pig_latin.push_str("hay");
					added += 3;
					pig_latin.push(a);
					println!(" pushed {:?} at index {}", last, index);
					// added=last_index - added;
				} else {
					println!("something went wrong");
				}				
			} else {
				pig_latin.push(last);
				pig_latin.push('a');
				pig_latin.push('y');

				pig_latin.push(a);	
				added += 2;		
			}
			to_pop = true;
			continue;
		}

		pig_latin.push(a);

		if to_pop || index == 0 {
			match pig_latin.pop() {
			 	Some(expr) => {
			 		last = expr;
			 		last_index = index;
			 	},
			 	None => panic!("{:?}", ),
			 }
			 println!("poped {:?}", last);
			to_pop = false;
		}
	}
	print!("{} piggy version {}", sentence, pig_latin);
}
