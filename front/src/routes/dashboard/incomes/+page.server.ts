import { apiUrl } from "$lib/config";
import type { Income } from "$lib/interfaces/incomes";
import { type RequestEvent, error, type Actions } from "@sveltejs/kit";

interface NewIncome {
  amount: number;
  description: string;
  processed: boolean;
  date: string;
}

export async function load({ cookies }: RequestEvent) {
  const url = apiUrl + '/income';

  const options = {
    method: 'GET',
    headers: {
      Authorization: `${cookies.get('token')}`
    }
  };

  const response = await fetch(url, options);

  if (response.ok) {
    const incomes: Income[] = await response.json();
    return {
      incomes: incomes
    };
  }

  error(500, "Server error " + response.status + ": " + await response.text());
}

export const actions: Actions = {
  create: async ({ request, cookies }: RequestEvent) => {
    const url = apiUrl + '/income';
    const income = await request.formData();

    let newIncome = {} as NewIncome;

    if (income.get('description')) {
      newIncome.description = income.get('description') as string;
    } else {
      return error(400, "Description is required");
    }

    if (income.get('amount')) {
      newIncome.amount = Number(income.get('amount'));
    } else {
      return error(400, "Amount is required");
    }

    if (income.get('date')) {
      newIncome.date = income.get('date') as string;
    } else {
      return error(400, "Date is required");
    }

    newIncome.processed = income.get('processed') === "on";

    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: `${cookies.get('token')}`
      },
      body: new URLSearchParams({
        description: newIncome.description,
        amount: newIncome.amount.toString(),
        processed: newIncome.processed.toString(),
        date: newIncome.date
      })
    };

    const response = await fetch(url, options);

    if (response.ok) {
      return { success: true };
    }

    error(500, "Server error " + response.status + ": " + await response.text());
  }
};
