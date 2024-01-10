FROM node:18-alpine AS base 

# WORKDIR /apps/frontend
# COPY . /apps/frontend/

 
FROM base AS runner
WORKDIR /apps/frontend
COPY . /apps/frontend/
EXPOSE 3000
CMD ["yarn", "run", "start"]