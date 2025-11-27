Name:           telegram-notifier
Version:        0.1.0
Release:        1%{?dist}
Summary:        CLI to send Telegram notifications or listen to incoming messages

License:        MIT
URL:            https://example.invalid/telegram-notifier
Source0:        %{name}-%{version}.tar.gz
BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig(openssl)

%description
A small CLI utility built with teloxide to send messages to a configured chat
or run in a listening mode that prints incoming messages (timestamp, chat_id,
username, text) to stdout.

%prep
%setup -q

%build
cargo build --release

%install
install -Dm755 target/release/telegram-notifier \
    %{buildroot}%{_bindir}/telegram-notifier
install -Dm644 etc/telegram-notifier/config.toml \
    %{buildroot}%{_sysconfdir}/telegram-notifier/config.toml

%files
%{_bindir}/telegram-notifier
%config(noreplace) %{_sysconfdir}/telegram-notifier/config.toml
%license LICENSE
%doc README.md

%changelog
* Tue Nov 26 2024 Telegram Notifier Bot <bot@example.invalid> - 0.1.0-1
- Initial package
