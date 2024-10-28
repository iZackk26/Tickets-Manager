import { Component } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Home from './pages/Home';
import Category from './pages/Category';
import SeatsView from './pages/SeatsView';
import BreadcrumbsWrapper from './components/BreadcrumbsWrapper';

class App extends Component {
  render() {
    return (
      <div className="w-screen">
        <BrowserRouter>
          <div className="flex flex-col items-center justify-center">
            <BreadcrumbsWrapper />
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/category" element={<Category />} />
              <Route path="/seats/:zone/:category" element={<SeatsView />} />
            </Routes>
          </div>
        </BrowserRouter>
      </div>
    );
  }
}

export default App;
