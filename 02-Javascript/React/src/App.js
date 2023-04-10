import docker from './docker.png';
import logo from './DevopsHobbis.jpg';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={docker} className="App-logo" alt="logo" />
        <p>
        Find Docker-File template.
        </p>
        <img src={logo} className="App-logo" alt="logo" />
         
        <a
          className="App-link"
          href="https://github.com/devopshobbies/docker-templates"
          target="_blank"
          rel="noopener noreferrer"
        >
         
DevOps Hobbies
        </a>
      </header>
    </div>
  );
}

export default App;
