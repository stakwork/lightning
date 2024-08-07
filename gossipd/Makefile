#! /usr/bin/make

GOSSIPD_HEADERS_WSRC := gossipd/gossipd_wiregen.h \
	gossipd/gossip_store_wiregen.h			\
	gossipd/gossipd.h				\
	gossipd/gossip_store.h				\
	gossipd/gossmap_manage.h			\
	gossipd/queries.h				\
	gossipd/txout_failures.h			\
	gossipd/sigcheck.h				\
	gossipd/seeker.h
GOSSIPD_HEADERS := $(GOSSIPD_HEADERS_WSRC)

GOSSIPD_SRC := $(GOSSIPD_HEADERS_WSRC:.h=.c)
GOSSIPD_OBJS := $(GOSSIPD_SRC:.c=.o)
$(GOSSIPD_OBJS): $(GOSSIPD_HEADERS)

# Make sure these depend on everything.
ALL_C_SOURCES += $(GOSSIPD_SRC)
ALL_C_HEADERS += $(GOSSIPD_HEADERS)
ALL_PROGRAMS += lightningd/lightning_gossipd

# Here's what lightningd depends on
LIGHTNINGD_CONTROL_HEADERS += gossipd/gossipd_wiregen.h
LIGHTNINGD_CONTROL_OBJS += gossipd/gossipd_wiregen.o

# Common source we use.
GOSSIPD_COMMON_OBJS :=				\
	bitcoin/chainparams.o			\
	common/amount.o				\
	common/autodata.o			\
	common/base32.o				\
	common/bech32.o				\
	common/bech32_util.o			\
	common/bigsize.o			\
	common/bip32.o				\
	common/channel_id.o			\
	common/cryptomsg.o			\
	common/daemon.o				\
	common/daemon_conn.o			\
	common/decode_array.o			\
	common/derive_basepoints.o		\
	common/dev_disconnect.o			\
	common/ecdh_hsmd.o			\
	common/features.o			\
	common/gossmap.o			\
	common/fp16.o				\
	common/status_wiregen.o			\
	common/key_derive.o			\
	common/lease_rates.o			\
	common/memleak.o			\
	common/msg_queue.o			\
	common/node_id.o			\
	common/onionreply.o			\
	common/per_peer_state.o			\
	common/ping.o				\
	common/psbt_open.o			\
	common/pseudorand.o			\
	common/random_select.o			\
	common/setup.o				\
	common/status.o				\
	common/status_wire.o			\
	common/subdaemon.o			\
	common/timeout.o			\
	common/utils.o				\
	common/utxo.o				\
	common/version.o			\
	common/wireaddr.o			\
	common/wire_error.o			\
	connectd/connectd_gossipd_wiregen.o

lightningd/lightning_gossipd: $(GOSSIPD_OBJS) $(GOSSIPD_COMMON_OBJS) $(BITCOIN_OBJS) $(WIRE_OBJS) $(HSMD_CLIENT_OBJS)

# The CSV was removed, but the generated files may live on!
clean: clean-old-gossip-gen

clean-old-gossip-gen:
	$(RM) gossipd_peerd_wiregen.*

include gossipd/test/Makefile
