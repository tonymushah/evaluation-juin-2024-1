import cookies from 'js-cookie';
import { CLIENT_TOKEN_KEY } from './tokenKey';

export default function getToken() {
	const token = cookies.get()[CLIENT_TOKEN_KEY];
	if (token) return token;
}
