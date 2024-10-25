interface PaymentType {
  cardNumber: number,
  cardHolderName: string,
  validationDate: string,
  expirationDate: string,
  cvv: number,
  paymentNetwork: string
}

export default PaymentType;
