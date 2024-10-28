import React, { Component, ReactNode } from 'react';

type BaseComponentProps = {
  onMount?: () => void;
  children?: ReactNode;
};

class BaseComponent<P = {}, S = {}> extends Component<BaseComponentProps & P, S> {
  componentDidMount() {
    if (this.props.onMount) {
      this.props.onMount();
    }
  }

  render() {
    return <>{this.props.children}</>;
  }
}

export default BaseComponent;
