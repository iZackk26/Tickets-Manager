import { useState } from 'react';
import DynamicPluginLoader from './DinamicPluginLoader';

function App() {
  const [loadPlugin, setLoadPlugin] = useState(false);

  const handleButtonClick = () => {
    setLoadPlugin(true);
  };

  return (
    <div className="flex items-center justify-center w-screen h-screen">
      <div>
        <button
          className="px-2 py-1 bg-red-500 rounded-xl overflow-hidden"
          onClick={handleButtonClick}
        >
          Press me to load a plug-in
        </button>

        {loadPlugin && <DynamicPluginLoader pluginName="Test" />}
      </div>
    </div>
  );
}

export default App;
