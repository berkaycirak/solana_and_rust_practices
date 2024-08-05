import {
	ActionGetResponse,
	ACTIONS_CORS_HEADERS,
} from '@solana/actions';

export const GET = (req: Request) => {
	const payload: ActionGetResponse = {
		icon: new URL('/ruggish.png', new URL(req.url).origin).toString(),
		label: 'Send Memo',
		description:
			'This is a simple actions that returns ruggish image',
		title: 'Memo Demo',
	};
	return Response.json(payload, {
		headers: ACTIONS_CORS_HEADERS,
	});
};

export const OPTIONS = GET;
