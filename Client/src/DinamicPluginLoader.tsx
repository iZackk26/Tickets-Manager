import { useEffect, useState } from 'react';

const DynamicPluginLoader = ({ pluginName }: { pluginName: string }) => {
  const [PluginComponent, setPluginComponent] = useState<React.FC | null>(null);

  useEffect(() => {
    import(`./plugins/${pluginName}`).then((module) => {
      setPluginComponent(() => module.default);
    });
  }, [pluginName]);

  if (!PluginComponent) {
    return <div>Cargando el plug-in...</div>;
  }

  return <PluginComponent />;
};

export default DynamicPluginLoader;
