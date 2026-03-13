import { apiUrl } from "$lib/config";
import type { Expense } from "$lib/interfaces/expenses";
import type { Income } from "$lib/interfaces/incomes";
import { type RequestEvent, error } from "@sveltejs/kit";

export async function load({ cookies }: RequestEvent) {
  const token = cookies.get('token');

  const options = {
    method: 'GET',
    headers: {
      Authorization: `${token}`
    }
  };

  const [resExpenses, resIncomes] = await Promise.all([
    fetch(`${apiUrl}/expense`, options),
    fetch(`${apiUrl}/income`, options)
  ]);

  if (resExpenses.ok && resIncomes.ok) {
    const expenses: Expense[] = await resExpenses.json();
    const incomes: Income[] = await resIncomes.json();

    return {
      expenses,
      incomes
    };
  }

  error(500, "Error loading dashboard data");
}
