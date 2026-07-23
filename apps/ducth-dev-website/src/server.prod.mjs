import express from 'express';

const app = express();
const port = Number(process.env.PORT ?? 3001);
app.get('/{*path}', (_req, res) => res.send('ducth-dev-website'));
app.listen(port);
