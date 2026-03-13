import { apiUrl } from '$lib/config';
import { type RequestEvent, error } from "@sveltejs/kit";

export async function DELETE({ params, cookies }: RequestEvent) {
  const url = apiUrl + '/income/' + params.id;
  const options = {
    method: 'DELETE',
    headers: {
      Authorization: `${cookies.get('token')}`,
    }
  };

  const response = await fetch(url, options);

  if (response.ok) {
    return new Response(null, { status: 204 });
  }

  if (response.status === 400) {
    return new Response(
      JSON.stringify({
        message: await response.text(),
        status: 400
      }),
    )
  }

  error(500, "Server error " + response.status + ": " + await response.text());
}

import { json } from '@sveltejs/kit';

export async function PATCH({ params, cookies, request }: RequestEvent) {
  const url = apiUrl + '/income/' + params.id;
  const formData = await request.formData();
  
  const options = {
    method: 'PATCH',
    headers: {
      'content-type': 'application/x-www-form-urlencoded',
      Authorization: `${cookies.get('token')}`,
    },
    body: new URLSearchParams({
      description: formData.get('description') as string,
      amount: formData.get('amount') as string,
      processed: formData.get('processed') as string,
      date: formData.get('date') as string,
    })
  };

  const response = await fetch(url, options);

  if (response.ok) {
    return json(await response.json());
  }

  error(response.status, "Server error " + response.status + ": " + await response.text());
}
