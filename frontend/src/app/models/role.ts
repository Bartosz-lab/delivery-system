export enum Role {
    Admin,
    Courier,
    PartnerUser,
}

export interface PartnerUser {
    PartnerUser: number;
}

export function isPartnerUser(object: any): object is PartnerUser {
    return 'PartnerUser' in object;
}
