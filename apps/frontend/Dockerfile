FROM node:18-alpine AS builder 

WORKDIR /frontend
COPY . .

 
FROM builder AS runner
WORKDIR /frontend
COPY --from=builder /frontend /frontend/
RUN yarn install
RUN yarn build
EXPOSE 3000
CMD ["yarn", "start"]