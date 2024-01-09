FROM node:18.17.0

WORKDIR /apps/frontend
COPY . /apps/frontend/


FROM base AS builder
WORKDIR /app
COPY --from=deps /apps/frontend/node_modules ./node_modules
COPY . .
RUN npm run build
 
FROM base AS runner
WORKDIR /apps/frontend
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
COPY --from=builder /apps/frontend/public ./public
COPY --from=builder --chown=nextjs:nodejs /apps/frontend/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /apps/frontend/.next/static ./.next/static
USER nextjs
EXPOSE 3000
ENV PORT 3000
CMD ["node", "server.js"]