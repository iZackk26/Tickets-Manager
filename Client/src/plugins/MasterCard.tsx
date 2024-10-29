import CreditCard from "../classes/CreditCard";

class MasterCard extends CreditCard {
  protected getCardBackground(): string {
    return "url('src/assets/Mastercard-Background.jpg')";
  }

  protected getCardLogo(): string {
    return "src/assets/Mastercard-Logo.png";
  }

  render() {
    return (
      <div className="w-96 h-56 text-black rounded-xl overflow-hidden relative shadow-lg bg-cover bg-center" style={{ backgroundImage: this.getCardBackground() }}>
        <div className="p-6 flex flex-col justify-between h-full">
          {this.renderInputs()}
          {this.renderCardLogo()}
        </div>
        <div className="absolute inset-0 bg-white opacity-5 pointer-events-none">
          <svg className="w-full h-full" xmlns="http://www.w3.org/2000/svg">
            <filter id="noise">
              <feTurbulence type="fractalNoise" baseFrequency="0.80" numOctaves="4" stitchTiles="stitch" />
            </filter>
            <rect width="100%" height="100%" filter="url(#noise)" />
          </svg>
        </div>
      </div>
    );
  }
}

export default MasterCard;
