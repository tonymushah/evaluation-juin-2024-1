import cookies from 'js-cookie';

export const CLIENT_TOKEN_KEY = 'equipe-token';

export default function getToken() {
	const token = cookies.get()[CLIENT_TOKEN_KEY];
	if (token) return token;
}
