import './App.css';
import 'editor-prose/styles.css';
import './i18n/i18n';
import { BrowserRouter } from 'react-router-dom';
import { ApolloProvider } from '@apollo/client';
import { graphqlClient } from './infrastructure/graphql/graphql-client';
import AppContent from './AppContent';

const App = () => {
  return (
    <ApolloProvider client={graphqlClient}>
      <BrowserRouter>
        <AppContent />
      </BrowserRouter>
    </ApolloProvider>
  );
};

export default App;
