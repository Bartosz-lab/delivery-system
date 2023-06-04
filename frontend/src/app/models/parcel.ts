export enum ParcelSize {
    S = 'S',
    M = 'M',
    L = 'L',
}

export interface Parcel {
    pickup_date: Date;
    recipient_address: {
        street: string,
        city: string,
        postal_code: string,
    }
    recipient_email: string;
    recipient_name: string;
    recipient_phone: string;
    size: ParcelSize,
    status_list: {
        status: string,
        time: string
    }[]
}
