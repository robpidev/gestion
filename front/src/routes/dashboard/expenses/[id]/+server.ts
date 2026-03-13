import { apiUrl } from '$lib/config';
import type { Expense } from '$lib/interfaces/expenses';
import { type RequestEvent, error, json } from "@sveltejs/kit";

export async function DELETE({ params, cookies }: RequestEvent) {
  const url = apiUrl + '/expense/' + params.id;
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
