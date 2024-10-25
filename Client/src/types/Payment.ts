interface PaymentType {
  cardNumber: number,
  cardHolderName: string,
  expirationDate: Date,
  cvv: number
}

export default PaymentType;
