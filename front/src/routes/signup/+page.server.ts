import { fail, redirect, error, type Actions, type RequestEvent } from "@sveltejs/kit";
import { NewUser } from "$lib/interfaces/user";
import { apiUrl } from "$lib/config";

export const actions: Actions = {
  create: async ({ request }: RequestEvent) => {
    const data = await request.formData();

    const user = new NewUser();
    let resp = user.userFromForm(data);
    if (resp) {
      return fail(400, { error: resp });
    }
    const url = apiUrl + '/auth/signup';
    const options = {
      method: 'POST',
      headers: { 'content-type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams({ ...user })
    };

    const response = await fetch(url, options);

    if (response.status === 500) {
      console.log(response.text());
      return fail(500, { error: 'Internal server error' });
    }

    if (response.status === 400) {
      return fail(400, { error: await response.text() });
    }

    if (response.status === 409) {
      return fail(409, { error: await response.text() });
    }

    if (response.status === 200) {
      redirect(303, '/login');
    }


    error(response.status, await response.text())

  }
}
