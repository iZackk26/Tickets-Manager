import PaymentType from "../types/Payment";

function TestPaymentPlugin({ cardNumber, cardHolderName, expirationDate, cvv }: PaymentType) {
  return (
    <div>
      <p>
        {cardNumber}
      </p>
      <p>
        {cardHolderName}
      </p>
      <p>
        {expirationDate.toString()}
      </p>
      <p>
        {cvv}
      </p>
    </div>
  )
}



export default TestPaymentPlugin;
