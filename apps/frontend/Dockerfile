# FROM node:18.17.0

# WORKDIR /apps/frontend
# COPY . /apps/frontend/


FROM base AS builder
WORKDIR /app
COPY --from=deps /apps/frontend/node_modules ./node_modules
COPY . .
RUN npm run build
 
FROM base AS runner
WORKDIR /apps/frontend
COPY . /apps/frontend/
CMD ["yarn", "run", "start"]