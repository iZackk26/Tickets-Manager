import PaymentType from "../types/Payment";

const TestPaymentPlugin2: PaymentType = {
  cardNumber: 18,
  cardHolderName: 'Arthur Morgan',
  expirationDate: new Date('2025-12-09'),
  cvv: 111
};

export default TestPaymentPlugin2;
