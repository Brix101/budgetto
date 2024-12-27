export default () => ({
  port: parseInt(process.env.PORT, 10),
  nodeEnv: process.env.NODE_ENV,
});
