use std::fmt::Debug;

use faststr::FastStr;
pub use metainfo::MetaInfo;
use metainfo::TypeMap;

use super::net::Address;

#[macro_export]
macro_rules! newtype_impl_context {
    ($t:ident, $cf:ident, $inner: tt) => {
        impl $crate::context::Context for $t {
            type Config = $cf;

            #[inline]
            fn rpc_info(&self) -> &$crate::context::RpcInfo<Self::Config> {
                self.$inner.rpc_info()
            }

            #[inline]
            fn rpc_info_mut(&mut self) -> &mut $crate::context::RpcInfo<Self::Config> {
                self.$inner.rpc_info_mut()
            }

            #[inline]
            fn extensions_mut(&mut self) -> &mut $crate::context::Extensions {
                self.$inner.extensions_mut()
            }

            #[inline]
            fn extensions(&self) -> &$crate::context::Extensions {
                self.$inner.extensions()
            }
        }
    };
}

pub struct RpcCx<I, Config> {
    pub rpc_info: RpcInfo<Config>,
    pub inner: I,
    pub extensions: Extensions,
}

#[derive(Default)]
pub struct Extensions(TypeMap);

impl std::ops::Deref for Extensions {
    type Target = TypeMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait Context {
    type Config: Send + Debug;

    fn rpc_info(&self) -> &RpcInfo<Self::Config>;
    fn rpc_info_mut(&mut self) -> &mut RpcInfo<Self::Config>;

    fn extensions(&self) -> &Extensions;
    fn extensions_mut(&mut self) -> &mut Extensions;
}

impl<I, Config> Context for RpcCx<I, Config>
where
    Config: Send + Debug,
{
    type Config = Config;

    #[inline]
    fn rpc_info(&self) -> &RpcInfo<Self::Config> {
        &self.rpc_info
    }

    #[inline]
    fn rpc_info_mut(&mut self) -> &mut RpcInfo<Self::Config> {
        &mut self.rpc_info
    }

    #[inline]
    fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    #[inline]
    fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }
}

impl<I, Config> std::ops::Deref for RpcCx<I, Config> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I, Config> std::ops::DerefMut for RpcCx<I, Config> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<I, Config> RpcCx<I, Config> {
    pub fn new(ri: RpcInfo<Config>, inner: I) -> Self {
        Self {
            rpc_info: ri,
            inner,
            extensions: Default::default(),
        }
    }
}

/// Endpoint contains the information of the service.
#[derive(Debug)]
pub struct Endpoint {
    /// `service_name` is the most important information, which is used by the service discovering.
    pub service_name: FastStr,
    pub address: Option<Address>,
    /// `tags` is used to store additional information of the endpoint.
    ///
    /// Users can use `tags` to store custom data, such as the datacenter name or the region name,
    /// which can be used by the service discoverer.
    pub tags: TypeMap,
}

impl Endpoint {
    /// Creates a new endpoint info.
    #[inline]
    pub fn new(service_name: FastStr) -> Self {
        Self {
            service_name,
            address: None,
            tags: TypeMap::default(),
        }
    }

    /// Gets the service name of the endpoint.
    #[inline]
    pub fn service_name_ref(&self) -> &str {
        &self.service_name
    }

    #[inline]
    pub fn service_name(&self) -> FastStr {
        self.service_name.clone()
    }

    /// Insert a tag into this `Endpoint`.
    #[inline]
    pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) {
        self.tags.insert(val);
    }

    /// Check if `Endpoint` tags contain entry
    #[inline]
    pub fn contains<T: 'static>(&self) -> bool {
        self.tags.contains::<T>()
    }

    /// Get a reference to a tag previously inserted on this `Endpoint`.
    #[inline]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.tags.get::<T>()
    }

    /// Sets the address.
    #[inline]
    pub fn set_address(&mut self, address: Address) {
        self.address = Some(address)
    }

    /// Gets the address.
    #[inline]
    pub fn address(&self) -> Option<Address> {
        self.address.clone()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Role {
    Client,
    Server,
}

#[derive(Debug)]
pub struct RpcInfo<Config> {
    pub role: Role,
    pub caller: Option<Endpoint>,
    pub callee: Option<Endpoint>,
    pub method: Option<FastStr>,
    pub config: Option<Config>,
}

impl<Config> RpcInfo<Config> {
    pub fn with_role(role: Role) -> RpcInfo<Config> {
        RpcInfo {
            role,
            caller: None,
            callee: None,
            method: None,
            config: None,
        }
    }

    pub fn new(
        role: Role,
        method: FastStr,
        caller: Endpoint,
        callee: Endpoint,
        config: Config,
    ) -> Self {
        RpcInfo {
            role,
            caller: Some(caller),
            callee: Some(callee),
            method: Some(method),
            config: Some(config),
        }
    }

    #[inline]
    pub fn role(&self) -> Role {
        self.role
    }

    #[inline]
    pub fn method(&self) -> Option<&FastStr> {
        self.method.as_ref()
    }

    #[inline]
    pub fn method_mut(&mut self) -> Option<&mut FastStr> {
        self.method.as_mut()
    }

    #[inline]
    pub fn caller(&self) -> Option<&Endpoint> {
        self.caller.as_ref()
    }

    #[inline]
    pub fn caller_mut(&mut self) -> Option<&mut Endpoint> {
        self.caller.as_mut()
    }

    #[inline]
    pub fn callee(&self) -> Option<&Endpoint> {
        self.callee.as_ref()
    }

    #[inline]
    pub fn callee_mut(&mut self) -> Option<&mut Endpoint> {
        self.callee.as_mut()
    }

    #[inline]
    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    #[inline]
    pub fn config_mut(&mut self) -> Option<&mut Config> {
        self.config.as_mut()
    }
}
