import { Component } from 'react';

type ButtonProps<T = any, R = any> = {
  text: string;
  onClick?: (args?: T) => R;
};

class Button<T = any, R = any> extends Component<ButtonProps<T, R>> {
  handleClick = () => {
    if (this.props.onClick) {
      this.props.onClick();
    }
  };

  render() {
    return (
      <button onClick={this.handleClick} className="bg-black text-white p-3 rounded-lg">
        {this.props.text}
      </button>
    );
  }
}

export default Button;
