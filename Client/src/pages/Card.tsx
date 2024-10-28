import MasterCard from "../plugins/MasterCard"

export default function Card() {
    return (
        <>
            <div>
                <MasterCard cardNumber={123455123} cardHolderName="Isaac Ramirez" validationDate="10/23" expirationDate="10/28" cvv={222}/>
            </div>
        </>
    )
}
