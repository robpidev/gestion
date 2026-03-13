// import cookies type
import { redirect, type RequestEvent, error } from '@sveltejs/kit';
import { apiUrl } from "$lib/config";
import type { UserToken } from "$lib/interfaces/user";
export async function load({ cookies, fetch }: RequestEvent) {

  const token = cookies.get('token');

  if (!token) {
    redirect(302, '/signin');
  }

  const url = apiUrl + '/user/refresh';
  const options = {
    method: 'GET',
    headers: {
      Authorization: token
    }
  };

  const response = await fetch(url, options);


  if (response.status === 200) {
    let userToken: UserToken = await response.json();
    cookies.set('token', userToken.token, { path: '/' });
    return {
      user: userToken.user
    }
  }

  if (response.status === 401) {
    return redirect(302, '/signin');
  }

  error(500, 'Internal server error: ' + await response.text());

}
