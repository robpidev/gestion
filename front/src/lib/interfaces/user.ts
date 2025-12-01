
export interface CreateUser {
  username: string;
  name: string;
  lastname: string;
  password: string;
}

export interface User {
  username: string;
  name: string;
  lastname: string;
  id: string;
}

export interface UserToken {
  user: User;
  token: string;
}



export class NewUser implements CreateUser {
  username: string;
  name: string;
  lastname: string;
  password: string;

  constructor() {
    this.username = "";
    this.name = "";
    this.lastname = "";
    this.password = "";
  }

  userFromForm(data: FormData): void | string {

    if (!data.get("name")) {
      return "No name";
    }

    if (data.get("name")?.toString().length === 0) {
      return "Name is empty";
    }

    if (!data.get("lastname")) {
      return "No lastname";
    }

    if (data.get("lastname")?.toString().length === 0) {
      return "Lastname is empty";
    }

    if (!data.get("username")) {
      return "No username";
    }

    if (data.get("username")?.toString().length === 0) {
      return "Username is empty";
    }

    if (!data.get("password")) {
      return "No password";
    }

    if (data.get("password")?.toString().length === 0) {
      return "Password is empty";
    }


    this.name = data.get("name") as string;
    this.lastname = data.get("lastname") as string;
    this.username = data.get("username") as string;
    this.password = data.get("password") as string;
  }
}
