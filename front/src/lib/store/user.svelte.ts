import { type User } from "$lib/interfaces/user";

export const userState = $state({
  user: null as User | null
});
