import { PartnerUser } from "./role";

export interface User {
    id: number;
    roles: (string | PartnerUser)[],
    token?: string;
}
