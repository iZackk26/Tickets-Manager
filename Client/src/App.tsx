import { Component } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Home from './pages/Home';
import Category from './pages/Category';

class App extends Component {
  render() {
    return (
      <div className="flex items-center justify-center w-screen">
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/category" element={<Category />} />
          </Routes>
        </BrowserRouter>
      </div>
    );
  }
}

export default App;
