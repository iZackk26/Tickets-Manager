import React, { Component } from "react";
import { LuNfc } from "react-icons/lu";

interface CreditCardState {
  cardNumber: string;
  cardHolderName: string;
  validationDate: string;
  expirationDate: string;
  cvv: string;
}

abstract class CreditCard extends Component<{}, CreditCardState> {
  constructor(props: {}) {
    super(props);

    // Inicializamos el estado
    this.state = {
      cardNumber: "",
      cardHolderName: "",
      validationDate: "",
      expirationDate: "",
      cvv: "",
    };
  }

  // Métodos para manejar los cambios en los inputs
  protected handleCardNumberChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ cardNumber: e.target.value });
  };

  protected handleCardHolderNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ cardHolderName: e.target.value });
  };

  protected handleValidationDateChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ validationDate: e.target.value });
  };

  protected handleExpirationDateChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ expirationDate: e.target.value });
  };

  protected handleCvvChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ cvv: e.target.value });
  };

  protected abstract getCardBackground(): string;

  protected abstract getCardLogo(): string;

  protected renderCardLogo() {
    return (
      <div className="absolute bottom-6 right-6">
        <img src={this.getCardLogo()} alt="Card Logo" className="w-24 pl-8 h-auto" />
      </div>
    );
  }

  protected renderInputs() {
    return (
      <>
        <div className="flex justify-between items-start">
          <LuNfc className="w-6 h-6 text-gray-500" />
          <div className="text-right">
            <p className="text-xs text-gray-700">Expires</p>
            <input
              type="text"
              value={this.state.expirationDate}
              onChange={this.handleExpirationDateChange}
              placeholder="MM/YY"
              className="bg-transparent border-none text-right text-black font-medium w-16 text-xs focus:outline-none"
            />
          </div>
        </div>
        <div className="space-y-4">
          <input
            type="text"
            value={this.state.cardNumber}
            onChange={this.handleCardNumberChange}
            placeholder="1234 5678 9012 3456"
            maxLength={19}
            className="bg-transparent border-none text-2xl tracking-wider font-semibold focus:outline-none w-full"
          />
          <div className="flex justify-between items-center">
            <div>
              <p className="text-xs text-gray-500">Card holder</p>
              <input
                type="text"
                value={this.state.cardHolderName}
                onChange={this.handleCardHolderNameChange}
                placeholder="John Doe"
                className="bg-transparent border-none text-lg text-black focus:outline-none w-full font-medium"
              />
            </div>
            <div>
              <p className="text-xs text-gray-500">CVV</p>
              <input
                type="text"
                value={this.state.cvv}
                onChange={this.handleCvvChange}
                placeholder="123"
                maxLength={3}
                className="bg-transparent border-none font-medium text-black focus:outline-none w-12 text-sm"
              />
            </div>
          </div>
          <div className="flex justify-between pt-4">
            <div>
              <p className="text-xs text-gray-500">Valid From</p>
              <input
                type="text"
                value={this.state.validationDate}
                onChange={this.handleValidationDateChange}
                placeholder="MM/YY"
                className="bg-transparent border-none font-medium text-black focus:outline-none w-16 text-xs"
              />
            </div>
          </div>
        </div>
      </>
    );
  }
}

export default CreditCard;
