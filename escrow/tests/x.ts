// () --> true
// )(- --> false
// ( --> false
// ) --> false

const checkValidation = (input: string) => {
	let counter = 0;

	for (let i = 0; i < input.length; i++) {
		if (counter < 0) return false;
		if (input[i] === `(`) {
			counter++;
		} else if (input[i] === ')') {
			counter--;
		}
	}

	if (counter === 0) {
		return true;
	} else {
		return false;
	}
};
