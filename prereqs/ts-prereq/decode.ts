import bs58 from 'bs58';

const kp = bs58.decode(
	'privateKey'
);

console.log('My Secret key is:', kp);
