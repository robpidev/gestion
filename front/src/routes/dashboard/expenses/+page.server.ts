import { apiUrl } from "$lib/config";
import type { Expense } from "$lib/interfaces/expenses";
import { type RequestEvent, error, type Actions } from "@sveltejs/kit";

interface NewExpense {
  amount: number;
  description: string;
  processed: boolean;
  date: string;
}

export async function load({ cookies }: RequestEvent) {

  const url = apiUrl + '/expense';

  const options = {
    method: 'GET',
    headers: {
      Authorization: `${cookies.get('token')}`
    }
  };

  const response = await fetch(url, options);

  if (response.ok) {
    const expeses: Expense[] = await response.json();
    return {
      expenses: expeses
    };
  }

  error(500, "Server error " + response.status + ": " + await response.text());

}

export const actions: Actions = {
  create: async ({ request, cookies }: RequestEvent) => {

    const url = apiUrl + '/expense';

    const expense = await request.formData();

    let newExpense = {} as NewExpense;

    // Check if all data are valid
    if (expense.get('description')) {
      newExpense.description = expense.get('description') as string;
    } else {
      return error(400, "Description is required");
    }

    if (expense.get('amount')) {
      const amount = expense.get('amount') as string;
      console.log(amount);
      newExpense.amount = Number(expense.get('amount'));
    } else {
      return error(400, "Amount is required");
    }

    if (expense.get('date')) {
      newExpense.date = expense.get('date') as string;
    } else {
      return error(400, "Date is required");
    }


    if (expense.get('processed') == "on") {
      newExpense.processed = true;
    } else {
      newExpense.processed = false;
    }

    console.log(newExpense);




    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: `${cookies.get('token')}`
      },
      body: new URLSearchParams(
        {
          description: newExpense.description,
          amount: newExpense.amount.toString(),
          processed: newExpense.processed.toString(),
          date: newExpense.date
        }
      )
    };

    const response = await fetch(url, options);

    if (response.ok) {
      return {
        success: true
      }
    }

    if (response.status === 400) {
      return error(400, "Server error " + response.status + ": " + await response.text());
    }

    if (response.status === 500) {
      return error(500, "Server error " + response.status + ": " + await response.text());
    }

    error(500, "Internal server error " + response.status + ": " + await response.text());

  }
}



