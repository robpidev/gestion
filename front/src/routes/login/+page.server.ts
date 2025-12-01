import { fail, redirect, error, type Actions, type RequestEvent } from "@sveltejs/kit";
import { NewUser, type UserToken } from "$lib/interfaces/user";
import { apiUrl } from "$lib/config";

export const actions: Actions = {
  login: async ({ cookies, request }: RequestEvent) => {
    const data = await request.formData();
    const url = apiUrl + '/auth/signin';
    const options = {
      method: 'POST',
      headers: { 'content-type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams({ username: data.get('username') as string, password: data.get('password') as string })
    };


    const response = await fetch(url, options);

    if (response.status === 200) {
      let data: UserToken = await response.json();
      console.log(data)
      cookies.set('token', data.token, { path: '/' });
      return {
        user: data,
        status: 200
      }
    }


    if (response.status === 500) {
      console.log(response.text());
      return fail(500, { error: 'Internal server error' });
    }

    error(response.status, await response.text());

  }
}

