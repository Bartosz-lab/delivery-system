export enum ParcelSize {
    S = 'S',
    M = 'M',
    L = 'L',
}

export enum Currency {
    PLN = 'PLN',
    EUR = 'EUR',
    USD = 'USD'
}

export interface Money {
    price: number;
    currency: Currency;
}

export type PriceListEmem = [ParcelSize, Money];